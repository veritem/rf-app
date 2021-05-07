import Link from 'next/link'

export function Nav() {
  return (
    <div className="fixed top-0 w-screen bg-primary shadow-md text-white text-center py-4">
      <div>
        <Link href="/">
          <a className="pr-2">Transactions</a>
        </Link>
        |
        <Link href="/cards">
          <a className="pl-2">Card</a>
        </Link>
      </div>
    </div>
  )
}
