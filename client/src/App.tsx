import {Routes , Route, useLocation} from 'react-router-dom'
import { TransitionGroup, CSSTransition } from "react-transition-group";
import HomePage from './pages/HomePage/HomePage';
import ProfilePage from './pages/ProfilePage/ProfilePage';
import StudentsPage from './pages/StudentsPage/StudentsPage';
import LoginPage from './pages/LoginPage/LoginPage';
import MainPage from './pages/MainPage/MainPage';

function App() {
  
  const duration = 0;
  let location = useLocation();

  return (
    <TransitionGroup>
      <CSSTransition
        timeout={duration}
        key={location.pathname}
        classNames="page"
        unmountOnExit        
      >
        <Routes location={location}>
          <Route path='/' element={<HomePage/>} >
            <Route path='profile' element={<ProfilePage/>}/>
            <Route path='students' element={<StudentsPage/>}/>
            <Route index element={<MainPage/>}/>
          </Route>
          <Route path='/login' element={<LoginPage/>} />
        </Routes>
      </CSSTransition>
    </TransitionGroup>
  )
}

export default App
