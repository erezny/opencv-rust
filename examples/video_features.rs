extern crate opencv;

use opencv::core;
use opencv::core::{mat};
use opencv::highgui;
use opencv::imgproc;
use opencv::features2d;
use opencv::features2d::Feature2D;

fn run() -> Result<(),String> {
    let window = "video capture";
    try!(highgui::namedWindow(window,1));
    let mut cam = try!(highgui::VideoCapture::for_device(0));
    let mut orb = try!(features2d::ORB::new(500, 1.2f32, 8, 31, 0, 2,
        features2d::ORB_HARRIS_SCORE, 31));
    loop {
        let mut frame = mat();
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            let mut gray = mat();
            try!(imgproc::cvtColor(&frame, &mut gray, imgproc::CV_BGR2GRAY, 0));
            let mut kps = ::opencv::sys::types::VectorOfKeyPoint::new();
            let mut desc = mat();
            let mask = mat();
            try!(orb.detectAndCompute(&gray, &mask, &mut kps, &mut desc, false));
            let mut display = mat();
            try!(features2d::drawKeypoints(&gray, &kps, &mut display,
                core::Scalar { data:[-1f64;4] }, features2d::DrawMatchesFlags_DEFAULT));
            try!(highgui::imshow(window, &display));
        }
        if try!(highgui::waitKey(10)) > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
