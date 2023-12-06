use opencv::{core, highgui, imgproc, prelude::*};

fn main() {
    let mut mat =
        core::Mat::new_rows_cols_with_default(200, 600, core::CV_8UC3, core::Scalar::all(255.))
            .unwrap();
    imgproc::put_text(
        &mut mat,
        "Hello OpenCV",
        core::Point::new(10, 50),
        imgproc::FONT_HERSHEY_SIMPLEX,
        1.0,
        core::Scalar::new(0., 0., 0., 0.),
        2,
        imgproc::LINE_AA,
        false,
    )
    .unwrap();
    let scale = 2.0;
    let mut scaled = core::Mat::default();

    imgproc::circle(
        &mut mat,
        core::Point::new(100, 100),
        50,
        core::Scalar::new(0., 0., 255., 0.),
        2,
        imgproc::LINE_AA,
        0,
    ).unwrap();

    imgproc::resize(
        &mat,
        &mut scaled,
        core::Size::new(0, 0),
        scale,
        scale,
        imgproc::INTER_LINEAR,
    ) .unwrap();


    highgui::named_window("window", highgui::WINDOW_KEEPRATIO).unwrap();
    highgui::named_window("resized", highgui::WINDOW_KEEPRATIO).unwrap();

    highgui::imshow("window", &mat).unwrap();
    highgui::imshow("resized", &scaled).unwrap();
    highgui::wait_key(0).unwrap();
}
