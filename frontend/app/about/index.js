import React, { Fragment } from 'react';


export default function About() {
    return (
        <div className="about">
            <p>duplex is an experimental speedcubing method</p>
            <p>the duplex method has two steps</p>
            <ul>
                <li>2x3x3 block</li>
                <li>last layer</li>
            </ul>
            <p>
                good systems for creating 2x3x3 blocks are already well known (F2L, RouxDFDB, Petrus, Heise) and will not be described here.
            </p>
            <p>
                the last layer is comprised of one look then a combination of two algs. the solutions for each case are generated from a list of about 40 algs.  an advantage of this approach is that since we can generate combinations from a known good list of algs, we can eliminate bad cases <span className="blue">almost*</span> entirely.
            </p>
            <p>
                since several solutions exist for each case, arranging them by TODO: sorting

                initially for learning, only the first alg has to be learnt - since it must reduce to a case you already know.
            </p>
            <p>
                subsets of DXLL can likely be incorporated into other systems
            </p>


            <pre>{`
add mirror F/B

give algs names w/ colours
click to see the alg / auto mirrored -> show middle case

rotate cube LL UI

subsets - strip first auf by rotating
select subset, generate cases that use smallest number of first algs OR shortest OR minimize auf
a>all solutions/a>
show inbetween state for solution
show cancelled version of alg if it exists
group by first alg
trainer: just alg mode

+Kirjava> that's something I'm wondering about DXLL. can you learn rules that allow you to solve cases you haven't learnt in a one by one fashion
+Kirjava> like something that applies to a group of cases
SPA mode website
provide algsets from 2ll

> duplex on cll

* add ability to select a subset, and list solutions that use the smallest quantity of first alg -> this is how to find recog patterns -> add colours to tell algs apart
* show shortest, reduce first alg

gamification (checkbox for learnt subset / case) |
done before: 270 / OLLCP
recommend cases / list recommended
you only have to learn the first alg
no bad algs™
leaderboard / worst case
stats / users / main

G:
0 [F, U, R, U', R', F'] 2 [F', L', U, L', D', L, U2, L', D, L2, U, F]
0 [F', U', L2, D', L, U2, L', D, L, U', L, F] 2 [F, R, U, R', U', F']
        speedswolving:
https://www.speedsolving.com/forum/threads/developing-a-better-ll-system.36791/

            `}</pre>
        </div>
    );
}
