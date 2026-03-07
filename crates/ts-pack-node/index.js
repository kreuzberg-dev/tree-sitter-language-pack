/* eslint-disable no-undef */

const { platform, arch } = process;

const PLATFORM_ARCH = `${platform}-${arch}`;

const NATIVE_BINDING_MAP = {
  'darwin-x64': 'ts-pack-node.darwin-x64.node',
  'darwin-arm64': 'ts-pack-node.darwin-arm64.node',
  'linux-x64': 'ts-pack-node.linux-x64-gnu.node',
  'linux-arm64': 'ts-pack-node.linux-arm64-gnu.node',
  'win32-x64': 'ts-pack-node.win32-x64-msvc.node',
  'win32-arm64': 'ts-pack-node.win32-arm64-msvc.node',
};

let native;

const bindingFile = NATIVE_BINDING_MAP[PLATFORM_ARCH];

if (bindingFile) {
  try {
    native = require(`./${bindingFile}`);
  } catch {
    // Fall back to unqualified name (local dev builds)
    native = require('./ts-pack-node.node');
  }
} else {
  // Fall back to unqualified name for unknown platforms
  native = require('./ts-pack-node.node');
}

module.exports = native;
