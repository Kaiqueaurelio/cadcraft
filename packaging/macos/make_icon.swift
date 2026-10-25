import AppKit
import Foundation

guard CommandLine.arguments.count == 2 else {
    fputs("usage: make_icon.swift output.png\n", stderr)
    exit(2)
}

let size = 1024
let canvas = CGFloat(size)
let bitmap = NSBitmapImageRep(
    bitmapDataPlanes: nil,
    pixelsWide: size,
    pixelsHigh: size,
    bitsPerSample: 8,
    samplesPerPixel: 4,
    hasAlpha: true,
    isPlanar: false,
    colorSpaceName: .deviceRGB,
    bytesPerRow: 0,
    bitsPerPixel: 0
)!
let graphics = NSGraphicsContext(bitmapImageRep: bitmap)!
NSGraphicsContext.saveGraphicsState()
NSGraphicsContext.current = graphics
graphics.imageInterpolation = .high

NSColor.clear.setFill()
NSBezierPath(rect: NSRect(x: 0, y: 0, width: canvas, height: canvas)).fill()

let background = NSBezierPath(roundedRect: NSRect(x: 32, y: 32, width: 960, height: 960), xRadius: 192, yRadius: 192)
NSColor(calibratedRed: 14/255.0, green: 24/255.0, blue: 42/255.0, alpha: 1).setFill()
background.fill()
NSColor(calibratedRed: 43/255.0, green: 65/255.0, blue: 94/255.0, alpha: 1).setStroke()
background.lineWidth = 12
background.stroke()

func point(_ x: CGFloat, _ y: CGFloat) -> NSPoint {
    NSPoint(x: canvas / 2 + x * canvas, y: canvas / 2 - y * canvas)
}

func strokeLine(_ points: [NSPoint], color: NSColor, width: CGFloat) {
    guard let first = points.first else { return }
    let path = NSBezierPath()
    path.lineCapStyle = .round
    path.lineJoinStyle = .round
    path.lineWidth = width
    path.move(to: first)
    for p in points.dropFirst() { path.line(to: p) }
    color.setStroke()
    path.stroke()
}

let blue = NSColor(calibratedRed: 56/255.0, green: 137/255.0, blue: 255/255.0, alpha: 1)
let cyan = NSColor(calibratedRed: 43/255.0, green: 220/255.0, blue: 220/255.0, alpha: 1)
let white = NSColor(calibratedRed: 244/255.0, green: 248/255.0, blue: 255/255.0, alpha: 1)
let orange = NSColor(calibratedRed: 255/255.0, green: 166/255.0, blue: 74/255.0, alpha: 1)

let cube: [(CGFloat, CGFloat)] = [
    (-0.30, -0.14), (0.0, -0.31), (0.30, -0.14), (0.30, 0.17),
    (0.0, 0.34), (-0.30, 0.17), (-0.30, -0.14)
]
strokeLine(cube.map { point($0.0, $0.1) }, color: blue, width: 22)
strokeLine([point(0.0, -0.31), point(0.0, 0.0)], color: cyan, width: 17)
strokeLine([point(-0.30, 0.17), point(0.0, 0.0), point(0.30, 0.17)], color: cyan, width: 17)
let d: [(CGFloat, CGFloat)] = [
    (-0.105, -0.155), (-0.105, 0.155), (-0.01, 0.155),
    (0.075, 0.105), (0.105, 0.0), (0.075, -0.105),
    (-0.01, -0.155), (-0.105, -0.155)
]
strokeLine(d.map { point($0.0, $0.1) }, color: white, width: 19)
strokeLine([point(0.15, -0.22), point(0.23, -0.265)], color: orange, width: 20)
let accent = NSBezierPath(ovalIn: NSRect(x: point(0.23, -0.265).x - 12, y: point(0.23, -0.265).y - 12, width: 24, height: 24))
orange.setFill()
accent.fill()

NSGraphicsContext.restoreGraphicsState()
guard let data = bitmap.representation(using: .png, properties: [:]) else {
    fputs("could not encode Decode CAD icon as PNG\n", stderr)
    exit(1)
}
try data.write(to: URL(fileURLWithPath: CommandLine.arguments[1]))
