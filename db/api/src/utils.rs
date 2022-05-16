use actix_web::Responder;

pub async fn print_type_of<T>(_: &T) -> impl Responder {
    //println!("{}", std::any::type_name::<T>())
    format!("{}", std::any::type_name::<T>())
}
