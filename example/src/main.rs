
fn main() {
    let l = vec![0, 1, 2, 3, 4, 5, 6, 7];

    let mut cy = l.iter().cycle();

    for _ in 0..80 {
        println!("{} {} {} {} {} {} {} {}", cy.next().unwrap(), cy.next().unwrap(),
        cy.next().unwrap(), cy.next().unwrap(),
        cy.next().unwrap(),cy.next().unwrap(),
        cy.next().unwrap(),cy.next().unwrap())
    }
}
