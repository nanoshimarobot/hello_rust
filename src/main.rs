use plotters::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
struct Vector2 {
    x: f32,
    y: f32,
    // s: &'a str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_width = 1080;
    let image_height = 720;

    let mut rng = thread_rng();
    let mut point_cloud: Vec<_> = (0..1000)
        .map(|_| Vector2 {
            x: rng.gen(),
            y: rng.gen(),
        })
        .collect();

    for i in 0..point_cloud.len() {
        //point_cloud[i] = Vector2 {x: rng.gen(), y: rng.gen()};
        println!("{:?}", point_cloud[i]);
    }

    let root = BitMapBackend::new("plot.png", (image_width, image_height)).into_drawing_area();
    root.fill(&WHITE);

    let caption = "Point";
    let font = ("monospace", 15);

    let mut chart: ChartContext<BitMapBackend, Cartesian2d<plotters::coord::types::RangedCoordf32, plotters::coord::types::RangedCoordf32>> = ChartBuilder::on(&root)
        .caption(caption, font.into_font())
        .margin(10)
        .x_label_area_size(16)
        .y_label_area_size(42)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    chart.configure_mesh().draw()?;


    // let mut point_cloud: Vec<Vector2>;
    // point_cloud.
    //let mut point_cloud=[Vector2{x:0f32, y:0f32}; 1000];

    // let mut v = Vector2 {x: 1.2, y: 3.4, s: ""};

    // {
    //     let s = String::from("asfd");
    //     v.s = s.as_str();
    // }

    //v.x;

    // chart.draw_series(
    //     point_cloud
    //         .iter()
    //         .map(|v| TriangleMarker::new((v.x, v.y), 8, &RED)),
    // )?;

    let point_series = PointSeries::<_,_,Circle<_,_>,_>::new(
        point_cloud.iter().map(|v|(v.x, v.y)), 5, &RED
    );

    chart.draw_series(point_series)?;

    Ok(())
    //
}

fn build_random_vector(mut point_cloud: Vec<Vector2>) -> Vec<Vector2> {
    let mut rng = thread_rng();
    for i in 0..point_cloud.len() {
        point_cloud[i].x = rng.gen();
        point_cloud[i].y = rng.gen();
    }
    point_cloud
}
