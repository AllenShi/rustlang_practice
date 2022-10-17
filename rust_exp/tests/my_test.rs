#[cfg(test)]
mod tests {
    use rust_exp_lib::utils::collections::dlist;  

    #[test]
    fn test_dlist() {
        let mut dlist : dlist::List<u32> = dlist::List::new();     
        dlist.push_front(1);     
        dlist.push_front(2);     
        dlist.push_front(3);      

        let dlist_iter = dlist.iter();     
        for d_val in dlist_iter {         
            println!("value is {}", d_val);     
        }      
    }
}
