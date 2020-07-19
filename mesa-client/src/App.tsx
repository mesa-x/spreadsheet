import * as React from 'react';
import ReactDataSheet from 'react-datasheet';
import "react-datasheet/lib/react-datasheet.css";
import { isNumber } from 'util';
import { HelloRequest } from "./static_codegen/proto/hello_world_pb";
import { GreeterClient } from "./static_codegen/proto/hello_world_pb_service";
export interface GridElement extends ReactDataSheet.Cell<GridElement, number> {
  value: number | string | null;
}

class MyReactDataSheet extends ReactDataSheet<GridElement, number> { }

interface AppState {
  grid: GridElement[][];
  messages: string[];
}

//You can also strongly type all the Components or SFCs that you pass into ReactDataSheet.
let cellRenderer: ReactDataSheet.CellRenderer<GridElement, number> = (props) => {
  var backgroundStyle: any = {}
  backgroundStyle.color = isNumber(props.cell.value) && props.cell.value && props.cell.value < 0 ? 'red' : undefined;
  backgroundStyle.textAlign = isNumber(props.cell.value) ? "right" : 'left'
  return (
    <td style={backgroundStyle} onMouseDown={props.onMouseDown} onMouseOver={props.onMouseOver} onDoubleClick={props.onDoubleClick} className="cell">
      {props.children}
    </td>
  )
}

export default class App extends React.Component<{}, AppState> {
  inLoop = false

  doLoop() {
    if (!this.inLoop) {
      this.inLoop = true
      setTimeout(() => {
        this.updateNumberValue(0, 0, x => x + 1)

        this.inLoop = false
        this.doLoop()
      }, 1000);
    }
  }

  updateNumberValue(row: number, col: number, theFunc: (arg: number) => number) {
    var grid = this.state.grid
    if (grid.length > row && grid[row].length > col) {
      var theVal = grid[row][col].value
      if (isNumber(theVal)) {
        theVal = theFunc(theVal)
        grid[row][col].value = theVal
        this.setState({ grid })
        return theVal
      }
    }

    return null
  }

  constructor(props: {}) {
    super(props)
    this.state = {
      grid: [
        [{ value: 1 }, { value: -3 }],
        [{ value: -2 }, { value: 4 }]
      ],
      messages: [""],
    }

  }
  private activateLasers() {
    this.updateNumberValue(1, 0, x => x + 4)
    var msg = new HelloRequest()
    msg.setName("David")
    var gc = new GreeterClient("http://" + window.location.hostname + ":3000/")
    gc.sayHello(msg, (err, resp) => {
      if (err !== null) {
        console.log("Got an error ", err)
      } else if (resp != null) {
        console.log("Got a response", resp)
        console.log("Message: ", resp.getMessage())
        this.state.messages.push(resp.getMessage() + " at " + (new Date()))

        
      }
    })
    var respStream = gc.helloOverAgain(msg)

    respStream.on('data', msg => {
      this.state.messages.push(msg.getMessage() + " count "+ msg.getCount() + " at " + (new Date()))
    })

    respStream.on('end', e => {console.log(e)})

  }


  render() {
    this.doLoop()
    return (
      <div>
        <button onClick={this.activateLasers.bind(this)}>
          Activate Lasers
        </button>
        <div>
          <table>
            <tbody>
              {this.state.messages.map((e, i) => <tr key={i}><td>{e}</td></tr>)}
            </tbody>
          </table>
        </div>
        <MyReactDataSheet
          data={this.state.grid}
          valueRenderer={(cell) => cell.value}
          onCellsChanged={changes => {
            const grid = this.state.grid.map(row => [...row])
            changes.forEach(({ cell, row, col, value }) => {
              var v2 = Number(value)
              grid[row][col] = { ...grid[row][col], value: !isNaN(v2) ? v2 : value }
            })
            this.setState({ grid })
          }}
          cellRenderer={cellRenderer}
        />
      </div>
    )
  }
}