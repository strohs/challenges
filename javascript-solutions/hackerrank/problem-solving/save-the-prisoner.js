// A jail has a number of prisoners and a number of treats to pass out to them. Their jailer decides the fairest way
// to divide the treats is to seat the prisoners around a circular table in sequentially numbered chairs. A chair
// number will be drawn from a hat. Beginning with the prisoner in that chair, one candy will be handed to each
// prisoner sequentially around the table until all have been distributed.
// The jailer is playing a little joke, though. The last piece of candy looks like all the others, but it tastes awful.
// Determine the chair number occupied by the prisoner who will receive that candy.
// For example, there are 4 prisoners and 6 pieces of candy. The prisoners arrange themselves in seats numbered 1 to 4.
// Let's suppose two is drawn from the hat. Prisoners receive candy at positions 2 3 4 1 2 3. The prisoner to be
// warned sits in chair number 3


/**
 *
 * @param n the number of prisoners     1 <= n <= 10^9
 * @param m the number of sweets        1 <= m <= 10^9
 * @param s the chair number to begin passing out sweets from 1 <= s <= n
 */
function saveThePrisoner(n, m, s) {
    const p = (s + (m - 1)) % n;
    return p === 0 ? n : p;
}

console.log( saveThePrisoner(5, 2, 1)); // 2
console.log( saveThePrisoner(5, 2, 2)); // 3
console.log( saveThePrisoner(4, 6, 2)); // 3