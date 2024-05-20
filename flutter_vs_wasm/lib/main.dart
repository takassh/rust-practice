import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_vs_wasm/src/rust/api/image_processing.dart';
import 'package:flutter_vs_wasm/src/rust/frb_generated.dart';
import 'package:image/image.dart' as img;

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final future = rootBundle.load('assets/logo.jpg');
  Uint8List? filteredImage;
  Uint8List? buffer;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('pic to edge')),
        floatingActionButton: FloatingActionButton(
          onPressed: () {
            if (buffer == null) return;
            setState(() {
              final start = DateTime.now();
              filteredImage = pic2Edge(buffer: buffer!);
              final elapsed = DateTime.now().difference(start);
              print('Elapsed: ${elapsed.inMilliseconds} ms');
            });
          },
          child: const Icon(Icons.filter),
        ),
        body: Center(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Expanded(
                child: FutureBuilder(
                  future: future,
                  builder: (context, snapshot) {
                    if (snapshot.connectionState == ConnectionState.done &&
                        snapshot.hasData) {
                      buffer = snapshot.data!.buffer.asUint8List();
                      return Image.memory(snapshot.data!.buffer.asUint8List());
                    } else {
                      return const CircularProgressIndicator();
                    }
                  },
                ),
              ),
              Expanded(
                child: filteredImage != null
                    ? Image.memory(filteredImage!)
                    : const Center(child: CircularProgressIndicator()),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// Uint8List pic2Edge({required Uint8List buffer}) {
//   final image = img.decodeJpg(buffer)!;
//   final bytes = Uint8List(image.width * image.height);
//   for (var x = 0; x < image.width; x++) {
//     for (var y = 0; y < image.height; y++) {
//       final pixel = image.getPixel(x, y);
//       final gray =
//           pixel.r * 299 / 1000 + pixel.g * 587 / 1000 + pixel.b * 114 / 1000;
//       bytes[x + y * image.height] = gray.toInt();
//     }
//   }

//   final grayImage = img.Image.fromBytes(
//     width: image.width,
//     height: image.height,
//     bytes: bytes.buffer,
//     numChannels: 1,
//   );

//   final gaussianImage = img.convolution(grayImage, filter: [
//     0.07511361,
//     0.1238414,
//     0.07511361,
//     0.1238414,
//     0.20417996,
//     0.1238414,
//     0.07511361,
//     0.1238414,
//     0.07511361,
//   ]);

//   final gaussianLaplacianImage =
//       img.convolution(gaussianImage, filter: [1, 1, 1, 1, -8, 1, 1, 1, 1]);

//   return img.encodePng(gaussianLaplacianImage);
// }
