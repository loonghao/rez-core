# rez-next-repository

[![Crates.io](https://img.shields.io/crates/v/rez-next-repository.svg)](https://crates.io/crates/rez-next-repository)
[![Documentation](https://docs.rs/rez-next-repository/badge.svg)](https://docs.rs/rez-next-repository)
[![License](https://img.shields.io/crates/l/rez-next-repository.svg)](LICENSE)
[![Build Status](https://github.com/loonghao/rez-next/workflows/CI/badge.svg)](https://github.com/loonghao/rez-next/actions)

[中文文档](README_zh.md) | [English](README.md)

**High-performance repository management for Rez Next** - Fast, reliable package discovery, repository scanning, and package operations.

## 🚀 Features

- **Fast Repository Scanning**: Optimized parallel scanning of package repositories
- **Package Discovery**: Intelligent package discovery with caching and indexing
- **Repository Management**: Comprehensive repository operations and management
- **Performance**: High-throughput scanning with intelligent caching
- **Compatibility**: Full compatibility with original Rez repository semantics

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rez-next-repository = "0.1.0"
```

## 🔧 Usage

### Basic Repository Operations

```rust
use rez_next_repository::{Repository, RepositoryManager};

// Create a repository manager
let mut manager = RepositoryManager::new();

// Add repositories
let repo = Repository::new("/path/to/packages".into());
manager.add_repository(repo);

// Scan for packages
let packages = manager.scan_all_repositories().await?;
println!("Found {} packages", packages.len());
```

### Package Discovery

```rust
use rez_next_repository::{RepositoryScanner, ScanOptions};

// Create scanner with options
let scanner = RepositoryScanner::new();
let options = ScanOptions::default()
    .with_parallel_scanning(true)
    .with_caching(true);

// Scan repository
let results = scanner.scan_repository("/path/to/packages", options).await?;
for result in results {
    println!("Found package: {} v{}", result.name, result.version);
}
```

### Repository Caching

```rust
use rez_next_repository::{RepositoryCache, CacheOptions};

// Create cache with options
let cache_options = CacheOptions::default()
    .with_ttl(3600) // 1 hour TTL
    .with_max_entries(10000);

let cache = RepositoryCache::new(cache_options);

// Cache operations are automatic during scanning
```

## 🏗️ Architecture

This crate provides comprehensive repository management capabilities:

- **Repository**: Core repository type with scanning and management
- **RepositoryManager**: Multi-repository management and coordination
- **RepositoryScanner**: High-performance parallel scanning
- **RepositoryCache**: Intelligent caching with TTL and LRU eviction

## 📊 Performance

Optimized for high-performance scenarios:
- Parallel repository scanning
- Intelligent caching and indexing
- Minimal memory footprint
- Fast package discovery operations

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](../../CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](../../LICENSE) file for details.

## 🔗 Related Crates

- [`rez-next-common`](../rez-next-common) - Common utilities and error handling
- [`rez-next-version`](../rez-next-version) - Version management and parsing
- [`rez-next-package`](../rez-next-package) - Package definitions and operations

---

Part of the [Rez Next](https://github.com/loonghao/rez-next) project - A high-performance Rust implementation of the Rez package manager.
