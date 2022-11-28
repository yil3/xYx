import { Row, Table } from "antd";
import { ColumnsType } from "antd/lib/table";
import { useEffect, useState } from "react";

/**
* @Author xYx
* @Date 2022-11-13 18:18:40
*/
export default function Token() {
  useEffect(() => { }, []);
  const [tableData, setTableData] = useState();
  const [params, setParams] = useState({});

  const tableTitle = () => (
    <Row></Row>
  )

  const columns: ColumnsType<any> = [{}];

  return (
    <>
      <Table rowKey={record => record.id} columns={columns} title={tableTitle} />
    </>
  );
};

