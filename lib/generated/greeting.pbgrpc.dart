//
//  Generated code. Do not modify.
//  source: greeting.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'greeting.pb.dart' as $0;

export 'greeting.pb.dart';

@$pb.GrpcServiceName('com.google.greeting.Greeter')
class GreeterClient extends $grpc.Client {
  static final _$sayHello = $grpc.ClientMethod<$0.HelloRequest, $0.HelloResponse>(
      '/com.google.greeting.Greeter/SayHello',
      ($0.HelloRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.HelloResponse.fromBuffer(value));
  static final _$sayHelloAgain = $grpc.ClientMethod<$0.HelloRequest, $0.HelloResponse>(
      '/com.google.greeting.Greeter/SayHelloAgain',
      ($0.HelloRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.HelloResponse.fromBuffer(value));

  GreeterClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options,
        interceptors: interceptors);

  $grpc.ResponseFuture<$0.HelloResponse> sayHello($0.HelloRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$sayHello, request, options: options);
  }

  $grpc.ResponseFuture<$0.HelloResponse> sayHelloAgain($0.HelloRequest request, {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$sayHelloAgain, request, options: options);
  }
}

@$pb.GrpcServiceName('com.google.greeting.Greeter')
abstract class GreeterServiceBase extends $grpc.Service {
  $core.String get $name => 'com.google.greeting.Greeter';

  GreeterServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.HelloRequest, $0.HelloResponse>(
        'SayHello',
        sayHello_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.HelloRequest.fromBuffer(value),
        ($0.HelloResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.HelloRequest, $0.HelloResponse>(
        'SayHelloAgain',
        sayHelloAgain_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.HelloRequest.fromBuffer(value),
        ($0.HelloResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.HelloResponse> sayHello_Pre($grpc.ServiceCall call, $async.Future<$0.HelloRequest> request) async {
    return sayHello(call, await request);
  }

  $async.Future<$0.HelloResponse> sayHelloAgain_Pre($grpc.ServiceCall call, $async.Future<$0.HelloRequest> request) async {
    return sayHelloAgain(call, await request);
  }

  $async.Future<$0.HelloResponse> sayHello($grpc.ServiceCall call, $0.HelloRequest request);
  $async.Future<$0.HelloResponse> sayHelloAgain($grpc.ServiceCall call, $0.HelloRequest request);
}
