import Head from 'next/head'
import { Fragment } from 'react'
import useSwr from 'swr'
import { fetcher } from '../lib/fetcher'

export default function Card() {
  const { data, error } = useSwr('http://localhost:8000/cards', fetcher, {
    refreshInterval: 500,
  })

  if (error) return <p>{{ error }}</p>
  if (!data) return <p>Loading...</p>

  return (
    <Fragment>
      <Head>
        <title>RFID | CARDS</title>
      </Head>

      <div className="h-screen font-sans bg-primary text-white flex justify-center items-center flex-col">
        <table className="border-collapse border border-gray-300 w-1/2">
          <thead>
            <tr>
              <th className="border border-gray-300 p-2">No</th>
              <th className="border border-gray-300 p-2">owner_names</th>
              <th className="border border-gray-300 p-2">card_number</th>
              <th className="border border-gray-300 p-2">date_created</th>
            </tr>
          </thead>

          <tbody>
            {data.map((tran, i) => (
              <tr key={'' + tran.id}>
                <td className="border border-gray-300 p-2">{i + 1}</td>
                <td className="border border-gray-300 p-2">
                  {tran.owner_names}
                </td>
                <td className="border border-gray-300 p-2">{tran.id}</td>
                <td className="border border-gray-300 p-2">
                  {new Date(tran.created_at).getFullYear()}-
                  {new Date(tran.created_at).getMonth()}-
                  {new Date(tran.created_at).getDate()}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </Fragment>
  )
}
