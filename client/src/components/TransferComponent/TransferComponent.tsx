import React, { useState , useEffect } from 'react';
import { Transfer } from 'antd';
import type { TransferDirection } from 'antd/es/transfer';

interface RecordType {
    key: string;
    title: string;
    description: string;
}

interface propsTransfer {
    listObjectsAvailable : any[],
    isOpenModal ?: boolean
    keyTab : string
    setStudentsSelectedInClass ?: any
}

export default function TransferComponent(props : propsTransfer) {
    const {
        listObjectsAvailable = [],
        keyTab,
        isOpenModal,
        setStudentsSelectedInClass
    } = props
    const listAvailable: RecordType[] = listObjectsAvailable.map((obj) => ({
        key: obj[`${keyTab}_id`],
        title: obj['full_name'],
        description: obj[`${keyTab}_id`] + "   " + obj['full_name'],
    }));

    useEffect(() => {
       setSelectedKeys([]);
       setTargetKeys([]); 
    },[listObjectsAvailable])

    const initialTargetKeys = listAvailable.map((item) => item.key);
    const [targetKeys, setTargetKeys] = useState(initialTargetKeys);
    const [selectedKeys, setSelectedKeys] = useState<string[]>([]);

    const onChange = (nextTargetKeys: string[], direction: TransferDirection, moveKeys: string[]) => {
        console.log('targetKeys:', nextTargetKeys);
        console.log('direction:', direction);
        console.log('moveKeys:', moveKeys);
        setTargetKeys(nextTargetKeys);
        setStudentsSelectedInClass(nextTargetKeys);
    };

    const onSelectChange = (sourceSelectedKeys: string[], targetSelectedKeys: string[]) => {
        console.log('sourceSelectedKeys:', sourceSelectedKeys);
        console.log('targetSelectedKeys:', targetSelectedKeys);
        setSelectedKeys([...sourceSelectedKeys, ...targetSelectedKeys]);
    };

    const onScroll = (direction: TransferDirection, e: React.SyntheticEvent<HTMLUListElement>) => {
        console.log('direction:', direction);
        console.log('target:', e.target);
    };

    return (
        <Transfer         
            dataSource={listAvailable}
            listStyle={{
                width: 300,
                height: 300,
            }}
            titles={[`${keyTab}s not class`, `${keyTab}s in class`]}
            targetKeys={targetKeys}
            selectedKeys={selectedKeys}
            onChange={onChange}
            onSelectChange={onSelectChange}
            onScroll={onScroll}
            render={(item) => item.description}
        />
    );
};
