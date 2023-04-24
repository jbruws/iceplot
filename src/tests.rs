#[test]
fn quadr() {
    let mut fc = FuncHandler::new();
    fc.expr = String::from("x * x + 4");
    fc.arg = 6.0;
    fc.calculate();
    assert_eq!(fc.res, String::from("40.0"));
}

#[test]
fn quadr_2() {
    let mut fc = FuncHandler::new();
    fc.expr = String::from("x * x + 4");
    fc.arg = -6.0;
    fc.calculate();
    assert_eq!(fc.res, String::from("40.0"));
}

#[test]
fn cubic() {
    let mut fc = FuncHandler::new();
    fc.expr = String::from("x * x * x + 4");
    fc.arg = -2.0;
    fc.calculate();
    assert_eq!(fc.res, String::from("-4.0"));
}

#[test]
fn linear() {
    let mut fc = FuncHandler::new();
    fc.expr = String::from("x + 4");
    fc.arg = -10.0;
    fc.calculate();
    assert_eq!(fc.res, String::from("-6.0"));
}

#[test]
fn trigonometry() {
    let mut fc = FuncHandler::new();
    fc.expr = String::from("cos(x)");
    fc.arg = 3.141;
    fc.calculate();
    assert!(fc.res.parse::<f64>().unwrap() + 1.0 < 0.1);
}

#[test]
fn trigonometry_2() {
    let mut fc = FuncHandler::new();
    fc.expr = String::from("cos(x) * cos(x) + sin(x) * sin(x)");
    fc.arg = 2.44;
    fc.calculate();
    assert!(fc.res.parse::<f64>().unwrap() - 1.0 < 0.1);
}
