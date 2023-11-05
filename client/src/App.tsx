import {Routes , Route, useLocation} from 'react-router-dom'
import { TransitionGroup, CSSTransition } from "react-transition-group";
import HomePage from './pages/HomePage/HomePage';
import ProfilePage from './pages/ProfilePage/ProfilePage';
import ListPage from './pages/ListPage/ListPage';
import LoginPage from './pages/LoginPage/LoginPage';
import MainPage from './pages/MainPage/MainPage';
import 'react-toastify/dist/ReactToastify.css';
import { ToastContainer } from 'react-toastify';
import PrivateRouter from './components/PrivateRouter/PrivateRouter';
import NotFoundPage from './pages/NotFoundPage/NotFoundPage';


function App() {
  
  const duration = 0;
  let location = useLocation();

  return (
    <>
      <ToastContainer/>
      <TransitionGroup>
        <CSSTransition
          timeout={duration}
          key={location.pathname}
          classNames="page"
          unmountOnExit        
        >
          <Routes location={location}>
            <Route 
              path='/' 
              element={
                <PrivateRouter>
                  <HomePage/>
                </PrivateRouter>
              } 
            >
              <Route path='profile' element={<ProfilePage/>}/>
              <Route path='list/:type' element={<ListPage/>}/>
              <Route index element={<MainPage/>}/>
            </Route>
            <Route path='/login' element={<LoginPage/>} />
            <Route path='/*' element={<NotFoundPage/>}/>
          </Routes>
        </CSSTransition>
      </TransitionGroup>    
    </>
  )
}

export default App
