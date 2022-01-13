// A Discrete Mathematics professor has a class of students. Frustrated with their lack of discipline, he decides
// to cancel class if fewer than some number of students are present when class starts. Arrival times go
// from on time (arrivalTime <= 0) to arrived late (arrivalTime > 0)
// Given the arrival time of each student and a threshhold number of attendees, determine if the class is canceled.


/**
 *  It must return YES if class is cancelled, or NO otherwise.
 * @param k - the threshold number of students
 * @param a - an array of integers representing arrival times
 */
function angryProfessor(k, a) {
    const onTimeStudents = a.filter(time => time <= 0).length;
    console.log('on time', onTimeStudents);
    if (onTimeStudents >= k) {
        // class not cancelled
        return 'NO'
    } else {
        // class cancelled
        return 'YES'
    }
}

console.log( angryProfessor(3, [-1, -3, 4, 2]) );