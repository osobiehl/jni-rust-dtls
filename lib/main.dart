library app;

import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:grpc/grpc.dart';

import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;
import 'package:path/path.dart' as path;
import 'package:minimal_ffi_grpc_example/generated/greeting.pbgrpc.dart';

import 'package:logging/logging.dart';

typedef DeviceLibInit = void Function();
typedef device_lib_start = ffi.Void Function();

void init(){
var libraryPath = path.join(Directory.current.path, 'lib', 'libnative.so');
print(libraryPath);
print(Directory.current.path) ;
if (Platform.isMacOS) { 
  libraryPath = path.join(Directory.current.path, 'libnative.dylib');
} else if (Platform.isWindows) { 
  libraryPath = path.join(Directory.current.path, 'libnative.dll');
}
final dylib = ffi.DynamicLibrary.open('libnative.so');
final DeviceLibInit hello = dylib
    .lookup<ffi.NativeFunction<device_lib_start>>('device_lib_start')
    .asFunction();
    hello();
}




Future<void> main(List<String> args) async {
    // Initialize logging
  print("hello");

  final log = Logger('ExampleLogger');
  Logger.root.level = Level.ALL; // defaults to Level.INFO
  Logger.root.onRecord.listen((record) {
    print('${record.level.name}: ${record.time}: ${record.message}');
  });

  init();
  print("after init");
  await Future.delayed(Duration(seconds: 100));
    final channel = ClientChannel(
    '127.0.0.1',
    port: 50051,
    options: ChannelOptions(
      credentials: ChannelCredentials.insecure(),
      codecRegistry:
          CodecRegistry(codecs: const [GzipCodec(), IdentityCodec()]),
    ),
  );
  print("made channel");

  final stub = GreeterClient(channel);
  print("made stub");


  final name = args.isNotEmpty ? args[0] : 'world';

  try {
   print("before sayhello");
    var response = await stub.sayHello(HelloRequest()..name = name);
    print('Greeter client received: ${response.message}');
    response = await stub.sayHelloAgain(HelloRequest()..name = name);
    print('Greeter client received: ${response.message}');
  } catch (e) {
    print('Caught error: $e');
  }
  await channel.shutdown();
}

class App extends StatelessWidget {
  final String title = 'Hello from Rust';

  final String greeting;

  App(this.greeting);

  Widget build(context) => MaterialApp(
      title: title,
      home: Scaffold(
        appBar: AppBar(title: Text(title)),
        body: Center(
            child:
                Column(mainAxisAlignment: MainAxisAlignment.center, children: [
          Text(greeting),
        ])),
      ));
}
