
/*
Two cats and a mouse are at various positions on a line. You will be given their starting positions. Your task is
to determine which cat will reach the mouse first, assuming the mouse doesn't move and the cats travel at equal speed.
If the cats arrive at the same time, the mouse will be allowed to move and it will escape while they fight.

You are given q queries in the form of x, y, and z representing the respective positions for cats A and B, and for
mouse C. Complete the function "catAndMouse" to return the appropriate answer to each query, which will be printed
on a new line.
    * If cat A catches the mouse first, print "Cat A".
    * If cat B catches the mouse first, print "Cat B".
    * If both cats reach the mouse at the same time, print "Mouse C" as the two cats fight and mouse escapes

For example, cat A is at position x=2 and cat B is at y=5. If mouse C is at position z=4, it is 2 units from cat A
and 1 unit from cat B. Cat B will catch the mouse.
 */


fn cat_and_mouse(x: i32, y: i32, z: i32) -> String {
    let x_dist = (x - z).abs();
    let y_dist = (y - z).abs();

    return {
        if x_dist < y_dist {
            "Cat A".to_string()
        } else if y_dist < x_dist {
            "Cat B".to_string()
        } else {
            "Mouse C".to_string()
        }
    };
}


fn main() {
    println!("{}", cat_and_mouse(1,2,3));
    println!("{}", cat_and_mouse(1,3,2));
}