// Prints 32 bit unsigned integer
#[no_mangle]
pub extern "C" fn printu32(num32 : u32)
{
   println!("{}", num32);
}
