import './MainPage.css'
import { FaPhone, FaAt, FaAddressBook } from "react-icons/fa";
import qr from "../../assets/img/qr.png"
export default function MainPage() {
  return (
    <div id="MainPage" className="background-container gradient-overlay">
      <body>
        <div className="row">
          <div className="col-md-6 info">
            <div className="motto">
              <span className="the-best">The Best</span> <br /> <span className="education">Education</span> <br /> <span className="manager">Manager</span>
            </div>
            <div className="naiyo">
              <div className="naiyo-title"> What Make Us different
              </div>
              <div className="naiyo-info">Web-based school management systems redefine the educational landscape with their innovative approach. By seamlessly integrating student, teacher, and classroom management, these systems provide educators with the tools they need to focus on teaching rather than administrative tasks. Features like automated attendance tracking, grade management, and secure data storage enhance efficiency and security, while instant communication channels foster collaboration among educators, students, and parents, creating a more informed and engaged educational ecosystem. These transformative systems stand out by streamlining scheduling, timetabling, and resource allocation, making educational institutions more agile and technology-driven.
                <br /> <br />Furthermore, they offer robust security and privacy features, ensuring the safety of sensitive academic data. In essence, web-based school management systems serve as the digital backbone of modern education, making schools more efficient, productive, and capable of meeting the evolving needs of the educational landscape.
              </div>
              <div className="row contact">
                <div className="col-md-4 QR">
                  <img className='qr-img' src={qr} 
                  alt="qr" />
                </div>
                <div className="col-md-8">
                  <div className="phone-info contact-info">
                    <FaPhone /> <span>+8401234567890</span>
                  </div>
                  <div className="gmail-info contact-info">
                    <FaAt /> <span>education-manager@gmail.com</span>
                  </div>
                  <div className="addr-info contact-info">
                    <FaAddressBook /> <span>50 Quang Trung district, Quy Nhon, Binh Dinh</span>
                  </div>
                </div>

              </div>

            </div>

          </div>

        </div>

      </body>
    </div>
  )
}
