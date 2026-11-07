use nalgebra::{DMatrix,DVector};
struct SecTaskInput {
    p: fn(f64) -> f64,
    q: fn(f64) -> f64,
    f: fn(f64) -> f64,
    alp0: f64,
    alp1: f64,
    bet0: f64,
    bet1: f64,
    A: f64,
    B: f64,
    left_border: f64,
    right_border: f64,
    n: i64,

}

fn our_func(x:f64,y:f64)->f64 { x*(y*y)-(x*x)+1.0}

fn euler(func: fn(f64,f64)->f64,a:f64, b:f64, c:f64,step: f64) -> f64 {
    let mut x:f64 = a;
    let mut y:f64 = c;

    let mut count: usize =0;
    let n = ((b-a) / step).round() as usize;
    while count < n {
        println!("x before {x}");
        y = y + (step*func(x,y));
        x = x+step;
        println!("y is {y},\n x after {x}");
        count += 1;
    } 
    y

    
}
fn euler_koshi(func: fn(f64,f64)->f64,a:f64, b:f64, c:f64,step: f64) -> f64 {
    let mut x:f64 = a;
    let mut y:f64 = c;

    let mut count: usize =0;
    let n = ((b-a) / step).round() as usize;
    while count < n {
        println!("x before {x}");
        let y_temp: f64 = y + (step*func(x,y));
        println!("y temp is {y_temp}");
        y = y+ (step * ((func(x,y)+func(x+step,y_temp))/2.0));
        x = x+step;
        println!("y is {y},\n x after {x}");
        count += 1;
    } 
    y    
}

fn runge_kutte(func: fn(f64,f64)->f64,a:f64, b:f64, c:f64,step: f64) -> f64 {
    let mut x:f64 = a;
    let mut y:f64 = c;

    let mut count: usize =0;
    let n = ((b-a) / step).round() as usize;
    while count < n {
        let y_fi = step * func(x,y);
        let y_se = step * func(x + (step/2.0), y + (y_fi/2.0));
        let y_th = step * func(x + (step/2.0), y + (y_se/2.0));
        let y_fo = step * func(x + step, y + y_th);
        y = y+(1.0/6.0)*(y_fi+(2.0*y_se)+(2.0*y_th)+y_fo);
        x = x+step;
        println!("y is {y},\n x after {x}");
        count += 1;
    } 
    y    
}

fn edge_sec_period(input: SecTaskInput) -> Option<DVector<f64>> {

    let h = (input.right_border-input.left_border)/input.n as f64;
    let find_x = |step: usize| input.left_border + (step as f64)*h;
    
    let n = input.n as usize;
    let mut m = DMatrix::<f64>::zeros(n+1,n+1);
    let mut d = DVector::<f64>::zeros(n+1);
    
    let (p, q, f) = (input.p,input.q,input.f);
    
    for i in 0..=(n-2){
        m[(i,i)] = (h*h)*q(find_x(i)) - h*p(find_x(i)) + 1.0;
        m[(i,i+1)] = h*p(find_x(i))-2.0;
        m[(i,i+2)] = 1.0;

        d[i] = (h*h) * f(find_x(i));
    }

    m[(n-1,0)] = input.alp0 * h - input.alp1;
    m[(n-1, 1)] = input.alp1;

    m[(n,n-1)] = -input.bet1;
    m[(n,n)] = input.bet0*h+input.bet1;

    d[n-1] = h*input.A;
    d[n] = h*input.B;

    if let Some(x) = m.lu().solve(&d) {
        println!("Solution x: {}", x);
        Some(x)
    } else {
        println!("Нельзя решить");
        None
    }
}
fn main() {
    let res = euler(our_func,4.0,5.0,0.7,0.1);
    println!("euler {}",res);
    let res = euler_koshi(our_func,4.0,5.0,0.7,0.1);
    println!("euler_koshi {}",res);
    let res = runge_kutte(our_func,4.0,5.0,0.7,0.1);
    println!("runge_kutte {}", res);
    // допустим вариант 12 к 2 задаче 
    let inp = SecTaskInput {
        p: |x: f64| -5.0*x,
        q: |x: f64| 3.0,
        f: |x: f64| 2.5,
        alp0: 0.0,
        alp1: 1.0,
        bet0: 0.5,
        bet1: 3.0,
        A: 3.2,
        B: 5.4,
        left_border: 0.7,
        right_border: 1.0,
        n: 4, // нет нигде в задаче этого числа, поэтому сам взял
    };
    let res = edge_sec_period(inp);
      
}
