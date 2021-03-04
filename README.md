# RuAnnoy

[![main](https://github.com/hanabi1224/RuAnnoy/actions/workflows/main.yml/badge.svg)](https://github.com/hanabi1224/RuAnnoy/actions/workflows/main.yml)
[![appveyor](https://ci.appveyor.com/api/projects/status/ux13ive7vhsg32el/branch/master?svg=true)](https://ci.appveyor.com/project/hanabi1224/ruannoy/branch/master)
[![travis](https://travis-ci.com/hanabi1224/RuAnnoy.svg?branch=master)](https://travis-ci.com/github/hanabi1224/RuAnnoy)
[![MIT License](https://img.shields.io/github/license/hanabi1224/RuAnnoy.svg)](https://github.com/hanabi1224/RuAnnoy/blob/master/LICENSE)
========
<!-- [![Build Status](https://img.shields.io/travis/hanabi1224/RuAnnoy/master.svg)](https://travis-ci.org/hanabi1224/RuAnnoy) -->

This library is a rust port of [spotify/annoy](https://github.com/spotify/annoy) , currently only index serving is supported.

### Install via [crates.io](https://crates.io/crates/ru_annoy)
[![Crates.io](https://img.shields.io/crates/v/ru_annoy.svg)](https://crates.io/crates/ru_annoy)
[![codecov](https://codecov.io/gh/hanabi1224/RuAnnoy/branch/master/graph/badge.svg?token=jVO7N0AVTH)](https://codecov.io/gh/hanabi1224/RuAnnoy)
```toml
# Cargo.toml
[dependencies]
ru_annoy = "0"
```

### Usage
```rust
use ru_annoy::*;

let index = AnnoyIndex::load(10, "index.ann", IndexType::Angular).unwrap();
let v0 = index.get_item_vector(0);
let nearest = index.get_nearest(v0.as_ref(), 5, -1, true);
```

## FFI support

### kotlin/java

It uses JNI bindings to rust crate and is ~5-10x faster than [pure java implementation](https://github.com/spotify/annoy-java) in [benchmark scenario](https://github.com/hanabi1224/RuAnnoy/tree/master/bench)
#### Install via [jitpack.io](https://jitpack.io/#hanabi1224/RuAnnoy)
[![Release](https://jitpack.io/v/hanabi1224/RuAnnoy.svg)](https://jitpack.io/#hanabi1224/RuAnnoy)
```gradle
repositories {
  mavenCentral()
  maven { url 'https://jitpack.io' }
}
  
dependencies {
  implementation 'com.github.hanabi1224:RuAnnoy:<tag>'
}
```
#### Usage
```kotlin
val index = AnnoyIndex.tryLoad("index.5d.ann", 5, IndexType.Angular)
```

### dotnet

| Runtimes                      | Nuget package                                                                                                                                 |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| RuAnnoy                       | [![NuGet version](https://buildstats.info/nuget/RuAnnoy)](https://www.nuget.org/packages/RuAnnoy)                                             |
| RuAnnoy-Batteries-Windows-x64 | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Windows-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Windows-x64) |
| RuAnnoy-Batteries-Linux-x64   | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Linux-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Linux-x64)     |
| RuAnnoy-Batteries-Darwin-x64  | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Darwin-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Darwin-x64)   |

#### Install via nuget
```xml
  <ItemGroup>
    <PackageReference Include="RuAnnoy" Version="*" />
    <PackageReference Include="RuAnnoy-Batteries-Windows-x64" Version="*" />
  </ItemGroup>
```
#### Usage
```csharp
var index = AnnoyIndex.Load("index.5d.ann", 5, IndexType.Angular);
```

## TODO
+ Index building support
+ CLI tool to build index from file
