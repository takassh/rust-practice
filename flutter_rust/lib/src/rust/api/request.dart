// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.25.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<ListResponse> getEndpointList({dynamic hint}) =>
    RustLib.instance.api.getEndpointList(hint: hint);

class ListResponse {
  final String ability;
  final String? berry;
  final String berryFirmness;

  const ListResponse({
    required this.ability,
    this.berry,
    required this.berryFirmness,
  });

  @override
  int get hashCode =>
      ability.hashCode ^ berry.hashCode ^ berryFirmness.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ListResponse &&
          runtimeType == other.runtimeType &&
          ability == other.ability &&
          berry == other.berry &&
          berryFirmness == other.berryFirmness;
}