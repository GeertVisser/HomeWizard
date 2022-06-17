use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;

//box plot
use plotlib::repr::BoxPlot;
use plotlib::view::CategoricalView;
use plotlib::style::BoxStyle;

use plotlib::style::{PointMarker, PointStyle}; //scatter plot
use plotlib::style::{LineJoin, LineStyle}; //line plot

pub fn plot_graph(line1: Vec<i32>, line2: Vec<i32>, line3: Vec<i32>, total: Vec<i32>) {
    line_test();
}


pub fn line_test() {
    let l1 = BoxPlot::from_slice(&[0., 2., 3., 2.])
          .style(&BoxStyle::new().fill("darkolivegreen"));

    let v = CategoricalView::new().add(l1);
    Page::single(&v).save("line.svg").expect("saving svg");
}


//visualize a vector of data
pub fn scatter_test(data1: Vec<(f64, f64)>, data2: Vec<(f64, f64)>) {
    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355"),
    ); // and a custom colour

    // We can plot multiple data sets in the same view
    let s2: Plot = Plot::new(data2).point_style(
        PointStyle::new() // uses the default marker
            .colour("#35C788"),
    ); // and a different colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();
}
