import 'dart:convert';
import 'dart:ffi';
import 'dart:io';

// Independent public ABI. No Flutter, package allocator or production engine.
void require(bool condition, String message) {
  if (!condition) throw StateError(message);
}

void main(List<String> args) {
  final library = args.isNotEmpty ? args.single : Platform.isWindows
      ? 'target/release/malphas_public_frame.dll'
      : Platform.isMacOS ? 'target/release/libmalphas_public_frame.dylib'
      : 'target/release/libmalphas_public_frame.so';
  final lib = DynamicLibrary.open(File(library).absolute.path);
  final create = lib.lookupFunction<Pointer<Void> Function(), Pointer<Void> Function()>('sample_frame_new');
  final length = lib.lookupFunction<UintPtr Function(), int Function()>('sample_frame_len');
  final data = lib.lookupFunction<Pointer<Uint8> Function(Pointer<Void>), Pointer<Uint8> Function(Pointer<Void>)>('sample_frame_data');
  final render = lib.lookupFunction<Int32 Function(Pointer<Void>, Uint32), int Function(Pointer<Void>, int)>('sample_frame_render');
  final free = lib.lookupFunction<Void Function(Pointer<Void>), void Function(Pointer<Void>)>('sample_frame_free');
  require(render(nullptr, 0) == -1, 'Null render must be rejected');
  require(data(nullptr) == nullptr, 'Null data must be null');
  free(nullptr);
  final frame = create();
  require(frame != nullptr, 'Allocation failed');
  try {
    final pointer = data(frame);
    final bytes = length();
    require(bytes == 16384 && pointer.address % 64 == 0, 'Layout mismatch');
    // This view borrows native memory. Do not retain it after finally/free.
    // All calls are synchronous; no read overlaps a Rust write.
    final view = pointer.asTypedList(bytes);
    require(render(frame, 0) == 0, 'Render failed');
    final checksum0 = view.fold<int>(0, (sum, value) => sum + value);
    require(view[0] == 0 && view[3] == 255, 'Initial pixel mismatch');
    require(render(frame, 1) == 0, 'Second render failed');
    final checksum1 = view.fold<int>(0, (sum, value) => sum + value);
    require(data(frame).address == pointer.address, 'Unexpected reallocation');
    require(view[0] == 1, 'Existing Dart view must observe the Rust write');
    require(checksum1 - checksum0 == 4096, 'Unexpected pixel delta');
    stdout.writeln(jsonEncode({'ok': true, 'bytes': bytes, 'alignment': 64,
      'same_allocation': true, 'borrowed_view_observes_write': true,
      'checksum_frame_0': checksum0, 'checksum_frame_1': checksum1}));
  } finally {
    free(frame);
  }
}
