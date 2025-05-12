fn main() {
    // Let can be used to desturcture a tuple into seperate variables. Add the mut keyword to x to make it mutable
    let (mut x, y) = (1, 2)
    x += 1

    // You can declare multiple variables in the same line
    let (x, y)
    // X is already declared so it doesn't have to be declared again. the .. indicates that the 4 is not something we care about
    (x, ..) = (3, 4)
    [.., y] = [1, 2]


}
