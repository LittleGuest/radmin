use captcha::Captcha;
use captcha::filters::Noise;

/// graph captcha
pub fn captcha() -> Captcha {
    let mut captcha = Captcha::new();
    captcha
        .add_chars(4)
        .apply_filter(Noise::new(0.2))
        // .apply_filter(Wave::new(1.0, 10.0))
        .view(110, 36)
    // .apply_filter(
    //     Cow::new()
    //         .min_radius(40)
    //         .max_radius(50)
    //         .circles(1)
    //         .area(Geometry::new(10, 100, 0, 0)),
    // )
    ;
    captcha
}
