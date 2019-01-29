import React, { Fragment, useState } from 'react';


export default function About() {

    const [almost, setAlmost] = useState(false);

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
                the last layer is comprised of one look then a combination of two algs. the solutions for each case are generated from a list of about 40 algs.  an advantage of this approach is that since we can generate combinations from a known good list of algs, we can eliminate bad cases
                {' '}{!almost ? (
                    <span
                        className="blue pointer"
                        onClick={() => { setAlmost(true); }}
                    >almost*</span>
                ) : (
                    <span className="blue">
                        *(about 3% of cases remain unsolved. this is still a work in progress - a better algset or alternative solution may resolve this. one thing you can do if you hit one of these cases is do a random alg from outside the list and it's likely you will come across another case you know ^_^)
                    </span>
                )}{' '}
                entirely.
            </p>
            <p>

                since several solutions exist for each case, arranging them by __TODO: sorting__

                the LL picker on the subsets page allows you to create custom subsets.

                initially for learning, only the first alg has to be learnt - since it must reduce to a case you already know.
            </p>
            <p>
                subsets of DXLL can likely be incorporated into other systems
            </p>
            <p>
                I managed to learn OLLCP with a <a href="https://www.speedsolving.com/wiki/index.php/OLLCP_(few_algs)" target="_blank">similar system</a> over the course of a few months. other methods from the same lineage are <a href="http://lar5.com/cube/270" target="_blank">Petrus270</a> and <a href="https://www.speedsolving.com/forum/threads/suneoll.23222/" target="_blank">SuneOLL</a>.
            </p>
            <p>
                <a href="http://www.github.com/kirjavascript/duplex">view the source</a>
            </p>

            <pre>
                {`
TODO:

add corner orientations (17 hours ago)
mention cases you already know (17 hours ago)

sort by (least transforms, shortest)
storage

why doesnt 1217172485964883 have a 1 alg solution (rotate during solving to get more LL indexes)
subsets - strip first auf by rotating
select subset, generate cases that use smallest number of first algs OR shortest OR minimize auf
group by first alg

can you learn rules that allow you to solve cases you haven't learnt in a one by one fashion, like something that applies to a group of cases
provide algsets from 2ll
add beginner method

* add ability to select a subset, and list solutions that use the smallest quantity of first alg -> this is how to find recog patterns
* show shortest, reduce first alg
                `}
            </pre>

        </div>

    );
}
