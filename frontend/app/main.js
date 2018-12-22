import React, { Fragment, useEffect } from 'react';
import { render } from 'react-dom';
import { BrowserRouter as Router, Route, Link } from 'react-router-dom';

import Algs, { AlgStore } from './algs';
import Solver from './solver/component';
import Explore from './explore';

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
        </Fragment>
    );
}

render((
    <Router>
        <AlgStore>
            <App />
        </AlgStore>
    </Router>
), document.body.appendChild(document.createElement('div')));
