import type { NextPage } from 'next'
import styles from './styles';
import { makeStyles } from '@mui/styles'
import Typography from '@mui/material/Typography'
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';

const useStyles = makeStyles(styles);

function createData(object: string, size: number, type: string, storageClass: string) {
  return { object, size, type, storageClass };
}

const rows = [
  createData('Frozen yoghurt', 159, '1.44mb', 'Standard'),
  createData('Ice cream sandwich', 237, '1.44mb', 'Standard'),
  createData('Eclair', 262, '1.44mb', 'Standard'),
  createData('Cupcake', 305, '1.44mb', 'Standard'),
  createData('Gingerbread', 356, '1.44mb', 'Standard'),
];

const Index: NextPage = () => {
  const classes = useStyles();

  return (
    <section className="home-container">
      <div className="home-content">
        <Typography variant="h3" gutterBottom>
          SaasLayerDev
          </Typography>
        <p>Objects are fundamental entities stored in Amazon S3.</p>

        <TableContainer className={classes.tableContainer} component={Paper}>
          <Table sx={{ minWidth: 650 }} aria-label="simple table">
            <TableHead>
              <TableRow>
                <TableCell>Object</TableCell>
                <TableCell align="right">Size</TableCell>
                <TableCell align="right">Type</TableCell>
                <TableCell align="right">Storage Class</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {rows.map((row) => (
                <TableRow
                  key={row.object}
                  sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
                >
                  <TableCell component="th" scope="row">
                    {row.object}
                  </TableCell>
                  <TableCell align="right">{row.size}</TableCell>
                  <TableCell align="right">{row.type}</TableCell>
                  <TableCell align="right">{row.storageClass}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </div>
    </section>
  )
}

export default Index
