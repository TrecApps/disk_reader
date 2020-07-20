pub trait Boot
{
    fn print_boot_level(&self);

    fn verify(&self) -> bool;
}