import './App.css';
import Navbar from './components/Navbar';
import Form from './components/Form';
import Home from './components/Home';
import React from "react";
import {
  BrowserRouter as Router,
  Route,
  Switch
} from "react-router-dom";
import About from './components/About'


function App() {
  return (
   <>
    <Router>
   <Navbar />
   
    <div>
      <Form />
    </div>
   
    <div className='contaner my-3'>
        <Switch>
        <Route exact path="/">
          <Home />
        </Route>
        <Route exact path="/about">
          <About />
        </Route>
    
      </Switch>
      </div>
    </Router>
   </>
  );
}

export default App;
