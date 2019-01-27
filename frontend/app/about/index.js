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
            <p>
                I managed to learn OLLCP with a <a href="https://www.speedsolving.com/wiki/index.php/OLLCP_(few_algs)" target="_blank">similar system</a> over the course of a few months. other methods from the same lineage are <a href="http://www.lar5.com/cube/270" target="_blank">Petrus270</a> and <a href="https://www.speedsolving.com/forum/threads/suneoll.23222/" target="_blank">SuneOLL</a>.
            </p>

            <pre>
                {`
TODO:

add mirror F/B
move parse error in desktop mode

why doesnt 1217172485964883 have a 1 alg solution
subsets - strip first auf by rotating
select subset, generate cases that use smallest number of first algs OR shortest OR minimize auf
group by first alg

can you learn rules that allow you to solve cases you haven't learnt in a one by one fashion, like something that applies to a group of cases
provide algsets from 2ll

* add ability to select a subset, and list solutions that use the smallest quantity of first alg -> this is how to find recog patterns
* show shortest, reduce first alg
                `}
            </pre>

        </div>

    );
}
