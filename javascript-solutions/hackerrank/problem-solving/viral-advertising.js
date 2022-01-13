// HackerLand Enterprise is adopting a new viral advertising strategy. When they launch a new product, they advertise
// it to exactly 5 people on social media.
// On the first day, half of those 5 people i.e. Floor(5/2) = 2  like the advertisement and share it with 3 of their
// friends. At the beginning of the second day Floor(5/2) * 3 = 2 * 3 = 6 people receive the advertisement.  Each day,
// Floor( recipients / 2) of the recipients share it with 3 of their friends on the following day. Determine how many
// people have liked the ad by the end of a given day, beginning with launch day as day 1


/**
 * return the cumulative number of people who have liked the add at the end of the given number of days
 * @param n - the number of days
 */
function viralAdvertising(n) {
    const likesPerDay = recipients => Math.floor( recipients / 2 );

    let totalLikes = 0;
    let recipients = 5;
    for (let i = 1; i <= n; i++) {
        const lpd = likesPerDay(recipients);
        totalLikes += lpd;
        recipients = lpd * 3;
    }
    return totalLikes;
}

console.log( viralAdvertising(3));