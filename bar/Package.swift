// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "bar",
    targets: [
        .executableTarget(
            name: "bar",
            dependencies: ["SystemMonitorObjC"],
            path: ".",
            sources: ["AppDelegate.swift", "main.swift"]
        ),
        .target(
            name: "SystemMonitorObjC",
            path: ".",
            sources: ["SystemMonitorApp-Bridging-Header.m"],
            publicHeadersPath: "."
        )
    ]
)
