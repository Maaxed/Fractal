
# Fractal

A fractal generator running on your gpu, 100% written in rust


## How to run the code

Clone the repository then open a console in the folder and execute the following command

```bash
cargo run --release
```

## Controls

- Left Click + Drag: Navigate around
- Right Click + Drag: Change the julia constant
- Scroll Wheel: Zoom in / out
- M: Mandelbrot set
- J: Julia set
- Keypad 3: Multibrot set with exponent 3
- T: Tricorn
- S: Burning ship fractal
- C: Mandelbrot method with z<sub>n+1</sub> = cos(z<sub>n</sub>) + 1 / c
- L: Lyapunov fractal
