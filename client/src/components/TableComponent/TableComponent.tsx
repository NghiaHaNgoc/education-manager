import { useState ,useRef} from 'react'
import { Button, Input, InputRef, Popconfirm, Space, Table } from "antd";
import { DeleteOutlined, EditOutlined, SearchOutlined } from "@ant-design/icons";
import Highlighter from 'react-highlight-words'
import { FilterConfirmProps } from "antd/es/table/interface";

interface propTable {
    typeList ?: string
    listData : any[],
    pageSize ?: number,
    onRow : any,
    setIsOpenModal ?: any
    setIsFormEdit ?: any
    handleDeleteStudent ?: any
}

export default function TableComponent(props : propTable) {
    const {
        typeList = '',
        listData = [],
        pageSize = 6,
        onRow,
        setIsOpenModal,
        setIsFormEdit,
        handleDeleteStudent
    } = props
    const [searchText, setSearchText] = useState('');
    const [searchedColumn, setSearchedColumn] = useState('');
    const searchInput = useRef<InputRef>(null);

    const handleSearch = (
        selectedKeys: string[],
        confirm: (param?: FilterConfirmProps) => void,
        dataIndex: any,
    ) => {
        confirm();
        setSearchText(selectedKeys[0]);
        setSearchedColumn(dataIndex);
    };

    const handleReset = (clearFilters: () => void) => {
        clearFilters();
        setSearchText('');
    };

    const getColumnSearchProps = (dataIndex: any): any => ({
        filterDropdown: ({ setSelectedKeys, selectedKeys, confirm, clearFilters, close }: any) => (
            <div style={{ padding: 8 }} onKeyDown={(e) => e.stopPropagation()}>
                <Input
                    ref={searchInput}
                    placeholder={`Search ${dataIndex}`}
                    value={selectedKeys[0]}
                    onChange={(e) => setSelectedKeys(e.target.value ? [e.target.value] : [])}
                    onPressEnter={() => handleSearch(selectedKeys as string[], confirm, dataIndex)}
                    style={{ marginBottom: 8, display: 'block' }}
                />
                <Space>
                    <Button
                        type="primary"
                        onClick={() => handleSearch(selectedKeys as string[], confirm, dataIndex)}
                        icon={<SearchOutlined />}
                        size="small"
                        style={{ width: 90 }}
                    >
                        Search
                    </Button>
                    <Button
                        onClick={() => clearFilters && handleReset(clearFilters)}
                        size="small"
                        style={{ width: 90 }}
                    >
                        Reset
                    </Button>
                    <Button
                        type="link"
                        size="small"
                        onClick={() => {
                            confirm({ closeDropdown: false });
                            setSearchText((selectedKeys as string[])[0]);
                            setSearchedColumn(dataIndex);
                        }}
                    >
                        Filter
                    </Button>
                    <Button
                        type="link"
                        size="small"
                        onClick={() => {
                            close();
                        }}
                    >
                        close
                    </Button>
                </Space>
            </div>
        ),
        filterIcon: (filtered: boolean) => (
            <SearchOutlined style={{ color: filtered ? '#1677ff' : undefined }} />
        ),
        onFilter: (value: any, record: any) =>
            record[dataIndex]
                .toString()
                .toLowerCase()
                .includes((value as string).toLowerCase()),
        onFilterDropdownOpenChange: (visible: boolean) => {
            if (visible) {
                setTimeout(() => searchInput.current?.select(), 100);
            }
        },
        render: (text: any) =>
            searchedColumn === dataIndex ? (
                <Highlighter
                    highlightStyle={{ backgroundColor: '#ffc069', padding: 0 }}
                    searchWords={[searchText]}
                    autoEscape
                    textToHighlight={text ? text.toString() : ''}
                />
            ) : (
                text
            ),
    });

    const handleShowInforDetail = () => {
        setIsFormEdit(true);
        setIsOpenModal(true);
    }

    const renderAction = () => {
        return (
            <div>
                <Popconfirm
                    title="Delete the task"
                    description={`Are you sure to delete this ${typeList}?`}
                    onConfirm={handleDeleteStudent}
                    okText="Yes"
                    cancelText="No"
                >
                    <DeleteOutlined
                        style={{ color: 'red', fontSize: '30px', cursor: 'pointer' }}
                    />
                </Popconfirm>
                <EditOutlined
                    style={{ color: 'orange', fontSize: '30px', cursor: 'pointer' }}
                    onClick={handleShowInforDetail}
                />
            </div>
        )
    }

    const columnsUser = [
        {
            title: 'Id',
            dataIndex: typeList === 'STUDENTS' ? 'student_id' : 'lecturer_id',
            render: (text: string) => <a>{text}</a>,
            width: 100,
            // sorter: (a: any, b: any) => a['student_id'].length - b['student_id'].length,
            ...getColumnSearchProps(typeList === 'STUDENT' ? 'student_id' : 'lecturer_id')
        },
        {
            title: 'Full name',
            dataIndex: 'full_name',
            render: (text: string) => <a>{text}</a>,
            width: 200,
            sorter: (a: any, b: any) => a['full_name'].length - b['full_name'].length,
            ...getColumnSearchProps('full_name')
        },
        {
            title: 'Birthdate',
            dataIndex: 'birth',
            render: (text: string) => <span>{text}</span>,
            sorter: (a: any, b: any) => a.birth.length - b.birth.length,
        },
        {
            title: 'Gender',
            dataIndex: 'gender',
            render: (text: string) => <span>{text}</span>,
        },
        {
            title: 'Address',
            dataIndex: 'address',
            render: (text: string) => <span>{text ? text : 'None'}</span>,

        },
        {
            title: 'Email',
            dataIndex: 'email',
            render: (text: string) => <span>{text ? text : 'None'}</span>,

        },
        {
            title: 'Phone',
            dataIndex: 'phone',
            render: (text: string) => <span>{text ? text : 'None'}</span>,

        },
        {
            title: 'Action',
            dataIndex: 'action',
            render: renderAction
        }
    ];

    return (
        <Table
            rowSelection={{
                type: "checkbox",
                onChange: (selectedRowKeys, selectedRows) => {
                    console.log(`selectedRowKeys: ${selectedRowKeys}`, 'selectedRows: ', selectedRows);
                },
            }}
            columns={columnsUser}
            dataSource={listData}
            onRow={onRow}
            bordered={true}
            pagination={{
                pageSize,
            }}
        />      
    )
}
