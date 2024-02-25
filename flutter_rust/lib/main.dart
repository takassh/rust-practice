
import 'package:flutter/material.dart';
import 'package:flutter_rust/src/rust/api/request.dart';
import 'package:flutter_rust/src/rust/api/simple.dart';
import 'package:flutter_rust/src/rust/api/custom_type.dart';
import 'package:flutter_rust/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
                'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
            Text(
                'Action: Call Rust `greetWrapedResult(name: "hey")`\nResult: `${greetWrapedResult(name: "hey")}`'),
            Text('Action: Call Rust `create()`\nResult: `${create()}`'),
            const FetchList(),
          ],
        ),
      ),
    );
  }
}

class FetchList extends StatefulWidget {
  const FetchList({super.key});

  @override
  State<FetchList> createState() => _FetchListState();
}

class _FetchListState extends State<FetchList> {
  late final Future<ListResponse> future;

  @override
  void initState() {
    future = getEndpointList();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
        future: future,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Text(snapshot.error.toString());
          }
          if (snapshot.data == null) {
            return const SizedBox();
          }

          final data = snapshot.data!;

          return Text(data.ability);
        });
  }
}
