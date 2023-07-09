#[macro_export]
macro_rules! get { 
    ($path: expr, $fun: expr) => ({
        actix_web::web::resource($path).route(actix_web::web::get().to($fun))
    })
}

#[macro_export]
macro_rules! post { 
    ($path: expr, $fun: expr) => ({
        actix_web::web::resource($path).route(actix_web::web::post().to($fun))
    })
}
