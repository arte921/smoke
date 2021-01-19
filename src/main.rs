use pixel_canvas::{Canvas, Color, input::MouseState};

const BREEDTE: i32 = 300;
const HOOGTE: i32 = 300;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f64,
    y: f64
}

#[derive(Debug, Clone, Copy)]
struct Simulatiepixel {
    dichtheid: f64,
    richting: Vector
}

fn main () {
    let canvas = Canvas::new(BREEDTE as usize, HOOGTE as usize)
        .render_on_change(false)
        .title("mandelbrot")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    let mut simulatiepixels: Vec<Vec<Simulatiepixel>> = Vec::new();

    for x in 0..BREEDTE as usize {
        simulatiepixels.push(Vec::new());
        for _ in 0..HOOGTE {
            simulatiepixels[x].push(Simulatiepixel {
                dichtheid: 0.0,
                richting: Vector {
                    x: 0.0,
                    y: 0.0
                }
            });
        }
    }

    canvas.render(move |muis, image| {
        let kopie = simulatiepixels.clone();

        for x in 0..BREEDTE as usize {
            for y in 0..HOOGTE as usize {
                if x > 0 && x < simulatiepixels.len() - 1 && y > 0 && y < simulatiepixels[0].len() - 1 {
                    for andere_x in x - 1 ..= x + 1 {
                        for andere_y in y - 1 ..= y + 1 {
                            simulatiepixels[andere_x][andere_y].dichtheid += kopie[andere_x][andere_y].dichtheid * if andere_x == x && andere_y == y {
                                1.5
                            } else if andere_y < y {
                                10.0
                            } else if andere_y > y {
                                0.1
                            } else {
                                1.0
                            }             
                        }
                    }
                }
            }
        }
        
        for (y, rij) in image.chunks_mut(BREEDTE as usize).enumerate() {
            for (x, pixel) in rij.iter_mut().enumerate() {
                if muis.x < BREEDTE && muis.y < HOOGTE {
                    simulatiepixels[muis.x as usize][muis.y as usize].dichtheid += 1.0;
                }

                simulatiepixels[x as usize][y as usize].dichtheid /= 1.0;
                            
                let kleurwaarde = simulatiepixels[x][y].dichtheid as u8;

                *pixel = Color {
                    r: kleurwaarde,
                    g: kleurwaarde,
                    b: kleurwaarde
                };
            }
        }
    });
}
