/*
Anna and Brian are sharing a meal at a restuarant and they agree to split the bill equally. Brian wants to order
something that Anna is allergic to though, and they agree that Anna won't pay for that item. Brian gets the check
and calculates Anna's portion. You must determine if his calculation is correct.

For example, assume the bill has the following prices: bill=[2,4,6]  Anna declines to eat item k = bill[2] which costs 6.
If Brian calculates the bill correctly, Anna will pay (2+4)/2 = 3. If he includes the cost of bill[2], he will
calculate (2+4+6)/2 = 6. In the second case, he should refund 3 to Anna.
 */

/**
 *  It should print Bon Appetit if the bill is fairly split. Otherwise, it should print the integer amount of money
 *  that Brian owes Anna.
 *
 *  bonAppetit has the following parameter(s):
 *      bill: an array of integers representing the cost of each item ordered
 *      k: an integer representing the zero-based index of the item Anna doesn't eat
 *      b: the amount of money that Anna contributed to the bill
 */
function bonAppetit( bill, k, b ) {
    let annaActual = bill
        .filter( (price,idx) => { return idx !== k } )
        .reduce( (acc,price) => { return acc + price } )

    annaActual = annaActual / 2
    if (annaActual === b) console.log("Bon Appetit")
    else console.log(b - annaActual)
}

function main() {
    const bill = [3,10,2,9];
    const k = 1;
    const b = 12;
    bonAppetit(bill,k,b)
}