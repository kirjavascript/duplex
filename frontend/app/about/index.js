import React, { Fragment, useState } from 'react';
import { useAlgs } from '#app/algs/store';

export default function About() {

    const [almost, setAlmost] = useState(false);

    const { algs } = useAlgs();

    return (
        <div className="about">
            <p><b>duplex</b> is an experimental speedcubing method</p>
            <p>the duplex method has two steps</p>
            <ul>
                <li>2x3x3 block</li>
                <li>last layer</li>
            </ul>
            <p>
                good systems for creating 2x3x3 blocks are already well known (F2L, RouxDFDB, Petrus, Heise) and will not be described here.
            </p>
            <p>
                the last layer is comprised of one look then a combination of two algs. the solutions for each case are generated from a list of {algs.length} algs.  an advantage of this approach is that since we can generate combinations from a known good list of algs, we can eliminate bad cases
                {' '}{!almost ? (
                    <span
                        className="blue pointer"
                        onClick={() => { setAlmost(true); }}
                    >practically*</span>
                ) : (
                    <span className="blue">
                        *(there are still 9 cases left unsolved with the default list. with so few, single algorithms can be learnt for them)
                    </span>
                )}{' '}
                entirely.
            </p>
            <p>
                initially, only the first alg has to be learnt - since it must reduce to a case you already know. even having to do a two look version at first should provide shorter / better solutions compared to traditional two look systems. sorting options should help finding recog patterns, and the LL picker on the subsets page allows you to create custom subsets.
            </p>
            <p>
                for example, if you select ELL and sort by 'reduce groups' you can see that 24 out of 29 cases start with the exact same first alg. this means that to learn those 24 cases, you only need to learn where the AUF is for each.
            </p>
            <p>
                when learning a subset, I recommend first learning the bare minimum to be able to solve the cases then later going over them again to learn them as full combos, and adding cancellations.
            </p>
            <p>
                I managed to learn OLLCP with a <a href="https://www.speedsolving.com/wiki/index.php/OLLCP_(few_algs)" target="_blank">similar system</a> over the course of a few months. other methods from the same lineage are <a href="http://lar5.com/cube/270" target="_blank">Petrus270</a> and <a href="https://www.speedsolving.com/forum/threads/suneoll.23222/" target="_blank">SuneOLL</a>. subsets of DXLL can likely be incorporated into other systems.
            </p>
            <p>
                <a href="http://www.github.com/kirjavascript/duplex">view the source</a>
                <br />
                <span className="blue pointer" onClick={() => { localStorage.clear(); location.reload(); }}>reset saved data</span>
            </p>
        </div>

    );
}
