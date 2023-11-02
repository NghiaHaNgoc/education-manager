import { Oval } from 'react-loader-spinner'
export default function LoadingComponent() {
  return (
    <Oval
        height={80}
        width={80}
        color="#4fa94d"
        wrapperStyle={{
            height : '100%',
            display : 'flex',
            justifyContent : 'center',
            alignItems : 'center'
        }}
        wrapperClass=""
        visible={true}
        ariaLabel='oval-loading'
        secondaryColor="#4fa94d"
        strokeWidth={2}
        strokeWidthSecondary={2}
    />
  )
}
