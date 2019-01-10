import React, { Fragment, useEffect } from 'react';
import { render } from 'react-dom';
import { BrowserRouter as Router, Route, Link } from 'react-router-dom';

import Algs from './algs';
import Solver from './solver';
import Explore from './explore';
import Subsets from './subsets';

import { AlgStore } from './algs/store';
import { CaseStore } from './subsets/store';
import { SolutionStore } from './solver/store';

const links = ['', 'explore', 'subsets', 'trainer', 'algs'];

function App(props) {

    return (
        <Fragment>
            <Solver />
            <Route component={({location}) => (
                <div className="menu">
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
            )} />

            <Route path="/algs" component={Algs} />
            <Route path="/explore" component={Explore} />
            <Route path="/subsets" component={Subsets} />
        </Fragment>
    );
}

render((
    [<App />, SolutionStore, CaseStore, AlgStore, Router]
        .reduce((children, Element) => <Element>{children}</Element>)
), document.body.appendChild(document.createElement('div')));
