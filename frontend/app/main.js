import React, { Fragment, useEffect } from 'react';
import { render } from 'react-dom';
import { BrowserRouter as Router, Route, Link } from 'react-router-dom';

import Algs, { AlgStore, useAlgs } from './algs';
import { startWorker, updateAlgs} from './solver';

const links = ['', 'explore', 'algs'];

function App(props) {

    // load worker, set initial algs
    // TODO: consecutive algs
    const { algs } = useAlgs();
    useEffect(() => {
        startWorker(() => {
            updateAlgs(algs);
        });
    }, []);

    return (
        <Fragment>
            <Route component={({location}) => (
                <div className="menu">
                    {links.map(link => (
                        <Link
                            className={
                                location.pathname === '/' + link ? 'active' : ''
                            }
                            key={link}
                            to={'/' + link}>
                            {link || 'home'}
                        </Link>
                    ))}
                </div>
            )} />

            <Route path="/algs" component={Algs} />
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
