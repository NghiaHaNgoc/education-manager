import { Button } from 'react-bootstrap';
import './Profile.css'
import { getProfileUser } from '../../services/userService';
import { updateProfileUser } from '../../services/userService';
import { useState, useEffect } from 'react';
import { toast } from 'react-toastify';
export default function ProfilePage() {

  const [showModal, setShowModal] = useState(false);

  const [userProfile, setUserProfile] = useState({
    full_name: '',
    password: '',
    birth: '',
    gender: '',
    email: '',
    phone: '',
    address: '',
  });

function isValidPhoneNumer(phoneNumber: string){
  const phoneRegex = /^[0-9]{10,11}$/;
  return phoneRegex.test(phoneNumber);
}
  useEffect(() => {
    // Lấy dữ liệu hồ sơ người dùng khi trang được tải
    const fetchData = async () => {
      try {
        const data = await getProfileUser();
        setUserProfile(data);
      } catch (error) {
        console.error('Error fetching user profile:', error);
      }
    };
    fetchData();
  }, []);

  const handleUpdateProfile = async () => {
    if (!isValidPhoneNumer(userProfile.phone)){
      toast.error('Invalid phone number format. Phone number must be 10 or 11 numbers.')
      return;
    }
    
    try {

      const birthPattern = /^\d{4}-\d{2}-\d{2}$/;
      if (!userProfile.birth.match(birthPattern)) {
        toast.error('Birth must follow format: YYYY-MM-DD');
        return;
      }
  
      const emailPattern = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$/;
      if (!userProfile.email.match(emailPattern)) {
        toast.error('Email is invalid');
        return;
      }

      const response = await updateProfileUser(userProfile);
      console.log('Update profile response:', response);
      toast.success('Updated successfully');
      handleCloseModal();
    } catch (error) {
      console.error('Error updating user profile:', error);
    }
  };

  const handleOpenModal = () => {
    setShowModal(true);
  };

  const handleCloseModal = () => {
    setShowModal(false);
  };

  

  return (
    <div id="ProfilePage">
      <div>
        <div className="form-group-title"> <h1>USER PROFILE</h1></div>
      </div>
      <div className="row">
        <div className="col-md-6 mt-2">
          <div className="form-group fullname-group">Full name:
            <input
              type="text"
              placeholder="Nguyen Van A"
              className="layout fullname-layout"
              value={userProfile.full_name}
              onChange={(e) =>{
                const name = e.target.value;
                if (/^[a-zA-Z\s]*$/.test(name)){
                setUserProfile({ ...userProfile, full_name: e.target.value })
              }}
            } 
            />
          </div>

          {/* <div className="form-group password-group">Password:
            <input 
                type="text"
                placeholder="Password"
                className="layout password-layout"
                value={userProfile.password}
                onChange={(e) => 
                  setUserProfile({...userProfile, password: e.target.value})
                }
             />
            </div> */}

          <div className="form-group birth-group">Birth:
            <input
              type="text"
              placeholder="2003-07-30"
              className="layout birth-layout"
              value={userProfile.birth}
              onChange={(e) =>
                setUserProfile({ ...userProfile, birth: e.target.value })
              }
            />
          </div>

          <div className="form-group gender-group">Gender:
            <input
              type="text"
              placeholder="Male"
              className="layout gender-layout"
              value={userProfile.gender}
              onChange={(e) => {
                // setUserProfile({...userProfile, birth: e.target.value})
              }}
              readOnly
            />
          </div>

          {/* <div className="form-group gender-group">Gender:
                  <label htmlFor="male" className="custom-radio-label">
                    <span className="custom-radio">
                      <input 
                      type="radio" 
                      name="gender"
                      value="male"
                      id="male"
                      checked={userProfile.gender === 'male'}
                      onChange={(e)=>
                        setUserProfile({...userProfile, gender: e.target.value})
                      }
                      />
                      <span className="checkmark"></span>
                    </span>
                    <span className="label">Male</span>
                  </label>
                  <label htmlFor="female" className="custom-radio-label">
                    <span className="custom-radio">
                      <input 
                      type="radio" 
                      name="gender"
                      value="female"
                      id="female"
                      checked={userProfile.gender === 'female'}
                      onChange={(e)=>
                        setUserProfile({...userProfile, gender: e.target.value})
                      }
                      />
                      <span className="checkmark"></span>
                    </span>
                    <span className="label">Female</span>
                  </label>
                  <label htmlFor="others" className="custom-radio-label">
                    <span className="custom-radio">
                      <input 
                      type="radio" 
                      name="gender"
                      value="others"
                      id="others"
                      checked={userProfile.gender === 'others'}
                      onChange={(e)=>
                        setUserProfile({...userProfile, gender: e.target.value})
                      }
                      />
                      <span className="checkmark"></span>
                    </span>
                    <span className="label">Others</span>
                  </label>
            </div> */}


        </div>


        <div className="col-md-6">
          <div className="form-group email-group">Email:
            <input
              type="email"
              placeholder="nguyenvana@gmail.com"
              className="layout email-layout"
              value={userProfile.email}
              onChange={(e) =>
                setUserProfile({ ...userProfile, email: e.target.value })
              }
            />
          </div>
          <div className="form-group phone-group">Phone:
            <input
              type="phone"
              placeholder="Your phone number"
              className="layout phone-layout"
              value={userProfile.phone}
              onChange={(e) =>
                setUserProfile({ ...userProfile, phone: e.target.value })
                
              }
              
            />
          </div>
          <div className="form-group address-group">Address:
            <input
              type="text"
              placeholder="50 Nguyen Hue district...."
              className="layout address-layout"
              value={userProfile.address}
              onChange={(e) =>
                setUserProfile({ ...userProfile, address: e.target.value })
              }
            />
          </div>

        </div>

      </div>

      <div className="form-group password-group password-button ">
        <button onClick={handleOpenModal}>CHANGE PASSWORD</button>

      </div>

      {showModal && (
        <div className="modal visible">
          <div className="modal-content">
            <span className="close" onClick={handleCloseModal}>
              &times;
            </span>
            <h4>CHANGE PASSWORD</h4>

            <input
              type="password"
              placeholder="Password"
              className="layout password-layout"
              value={userProfile.password}
              onChange={(e) =>
                setUserProfile({ ...userProfile, password: e.target.value })
              }
            />
            <button className="pw-button" onClick={handleUpdateProfile}>Update Password</button>
          </div>
        </div>
      )}

      <div className='update-button'>
        <Button style={{ width: "15%" }} variant="danger" onClick={handleUpdateProfile}>
          Update Profile
        </Button>
      </div>

    </div>
  );
}
