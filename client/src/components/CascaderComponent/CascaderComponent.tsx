import { Cascader } from 'antd';
import type { DefaultOptionType } from 'antd/es/cascader';
import {SetStateAction , Dispatch} from 'react'

interface Option {
    value: string;
    label: string;
    children?: Option[];
    disabled?: boolean;
}

interface propsCascader {
    options : Option[],
    placeholder ?: string,
    setCodeClass  : Dispatch<SetStateAction<string>>
}

export default function CascaderComponent(props : propsCascader) {

    const {
        options = [],
        placeholder = '',
        setCodeClass 
    } = props

    // const options: Option[] = [
    //     {
    //         value: 'zhejiang',
    //         label: 'Zhejiang',
    //     },
    // ];

    const onChange = (value: any, selectedOptions: Option[]) => {
        if(value){
            setCodeClass(value[0]);
        }else{
            setCodeClass('');
        }
        console.log(value, selectedOptions);
    };

    const filter = (inputValue: string, path: DefaultOptionType[]) =>
        path.some(
            (option) => (option.label as string).toLowerCase().indexOf(inputValue.toLowerCase()) > -1,
        );


    return (
        <Cascader
            options={options}
            onChange={onChange}
            placeholder={placeholder}
            showSearch={{ filter }}
            onSearch={(value) => console.log(value)}
        />
        
    )
}
