import React, { Fragment, useEffect } from 'react';
import { render } from 'react-dom';
import { BrowserRouter as Router, Route, Link } from 'react-router-dom';

import About from './about';
import Algs from './algs';
import Solver from './solver';
import Subsets from './subsets';
import Trainer from './trainer';

import { AlgStore } from './algs/store';
import { CaseStore } from './subsets/store';
import { SolutionStore } from './solver/store';

const links = ['', 'subsets', 'trainer', 'algs'];

function App(props) {
    return (
        <Fragment>
            <Route
                component={({location}) => (
                    <div className="menu" ref={(node) => {
                        if (node) {
                            const { height } = node.getBoundingClientRect();
                            document.body.style.marginTop = height + 'px';
                        }
                    }}>
                        {links.map(link => (
                            <Link
                                className={
                                    location.pathname === '/' + link ? 'active' : ''
                                }
                                key={link}
                                to={'/' + link}>
                                {link || 'about'}
                            </Link>
                        ))}
                    </div>
                )}
            />
            <Route exact path="/" component={About} />
            <Route path="/algs" component={Algs} />
            <Route path="/subsets" component={Subsets} />
            <Route path="/trainer" component={Trainer} />
        </Fragment>
    );
}

if (typeof WebAssembly !== 'object') {
    document.body.innerHTML = 'this website requires WebAssembly';
} else {
    render((
        [<App />, Solver, SolutionStore, CaseStore, AlgStore, Router]
        .reduce((children, Element) => <Element>{children}</Element>)
    ), document.body.appendChild(document.createElement('div')));
}
