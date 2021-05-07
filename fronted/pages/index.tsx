import Head from 'next/head'
import { Fragment } from 'react'
import useSwr from 'swr'
import { fetcher } from '../lib/fetcher'

export default function Home() {
  const { data, error } = useSwr(
    'http://localhost:8000/transactions',
    fetcher,
    { refreshInterval: 500 }
  )

  if (!data) return <p>Loading...</p>
  if (error) return <p>{{ error }}</p>

  return (
    <Fragment>
      <Head>
        <title>RFI | Transactions</title>
      </Head>
      <div className="h-screen pb-4  font-sans bg-primary text-white flex justify-center items-center flex-col">
        <h1 className="font-black text-3xl mb-6 mt-20">RFID Transactions</h1>
        <table className="border-collapse border border-gray-300 w-1/2">
          <tr>
            <th className="border border-gray-300 p-2">No</th>
            <th className="border border-gray-300 p-2">Initial Balance</th>
            <th className="border border-gray-300 p-2">Transaction fee</th>
            <th className="border border-gray-300 p-2">Customer id</th>
          </tr>
          {data.map((tran, i) => (
            <tr key={'' + tran.id}>
              <td className="border border-gray-300 p-2">{i + 1}</td>
              <td className="border border-gray-300 p-2">
                {tran.initial_balance}
              </td>
              <td className="border border-gray-300 p-2">
                {tran.transport_fare}
              </td>
              <td className="border border-gray-300 p-2">{tran.card_id}</td>
            </tr>
          ))}
        </table>
      </div>
    </Fragment>
  )
}
